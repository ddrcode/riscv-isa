import { readFileSync, writeFileSync } from "fs";

const valid_masks = new Set([
    "0xfe00707f",
    "0x707f",
    "0x7f",
    // "0xfc00707f"
]);

const GEN_CODE_WARN = "// Code in this file is generated\n// Don't modify this file manually!\n";

const loadData = (file) => {
    const data = readFileSync(file, { encoding: 'utf8', flag: 'r' });
    return JSON.parse(data);
}

const processFile = (isa) => {
    return Object.entries(isa)
        .map(processEntry)
        .filter(x => !!x)
        .map((obj, id) => {
            obj.id = id;
            return obj;
        })
};

const is32bit = encoding => encoding.slice(-2) === "11";

const processEntry = ([key, val]) => {
    const { mask, encoding, match, extension } = val;

    if (!is32bit(encoding)) {
        console.log(`Unprocessed instruction: ${key}`);
        return null;
    }

    const res = {
        name: key,
        extension: extension.map(e => e.replace(/^.+_/, "")),
        arch: extension[0].indexOf("rv64") == 0 ? 64 : 32,
        key: valid_masks.has(mask) ? parseKey(encoding) : null,
        match,
        mask
    };

    return res;
};

const parseKey = encoding => {
    return (parseInt(encoding.replace(/-/g, ""), 2) >> 2);
};

const processDef = def => {
    const name = def.mnemonic;
    const mnemonic = name[0].toUpperCase() + name.slice(1);
    const key = def.key.toString(16).padStart(4, "0");
    const ext_raw = def.extension[0];
    const ext = String(ext_raw[0]).toUpperCase() + ext_raw.slice(1);
    const rust = `        (0x${key}, ("${name}", M::${mnemonic}, EXT::${ext}, ${def.arch})),`;
    return rust;
}

const toFirstUppercase = (str) => {
    return String(str[0]).toUpperCase() + String(str.slice(1));
};

// const getRustCode = entries => {
//     const header = `
// // this is generated code, don't modify it manually!
//
// use once_cell::sync::Lazy;
// use std::collections::HashMap;
// use crate::model::{RISCVExtension as EXT, Mnemonic as M};
//
// type Row = (&'static str, M, EXT, u8);
//
// pub (crate) static INSTRUCTIONS: Lazy<HashMap<u16, Row>> = Lazy::new(|| {
//     HashMap::from([
// `;
//
//     const footer = `
//     ])
// });
// `;
//
//     const lines = entries
//         .map(processDef)
//         .reduce((str, line) => `${str}\n${line}`, "");
//
//
//     return header + lines + footer;
// }


const generateArrayEntry = ({ name, extension, arch, mask, match, key }) => {
    const ext = toFirstUppercase(extension[0]);
    const mnemonic = toFirstUppercase(name);
    const k = key ? `Some(0x${key.toString(16).padStart(4, "0")})` : "None";
    return `InstructionDef::new("${name}", M::${mnemonic}, E::${ext}, ${arch}, ${mask}, ${match}, ${k})`;
};

const generateRustArray = entries => {
    const header = `pub(crate) static INSTRUCTIONS: Lazy<[InstructionDef; ${entries.length}]> = Lazy::new(|| {[\n`;
    const code = entries
        .map(generateArrayEntry)
        .reduceRight((str, line) => line + ",\n" + str, "");
    return header + code + "\n]});";
}

const generateMnemonicsEnum = entries => {
    const code = entries.map(({name}, idx) => {
        const mnem = toFirstUppercase(name);
        return `    ${mnem} = ${idx},`
    }).join("\n");
    return `${GEN_CODE_WARN}
#[non_exhaustive]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mnemonic {
${code}
}
`;
}

const generateExtensionsEnum = entries => {
    const ext = new Set(entries.map(({extension}) => toFirstUppercase(extension[0])));
    const code = [...ext.keys().map(e => `    ${e},`)].sort().join("\n");
    return `${GEN_CODE_WARN}
#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RISCVExtension {
${code}
    Custom
}`;
};



const generateRustCode = (entries) => {
    const array = generateRustArray(entries);

    return `${GEN_CODE_WARN}
use once_cell::sync::Lazy;
use crate::data::InstructionDef;
use crate::model::{RISCVExtension as E, Mnemonic as M};

${array}
    `;
}

try {

    if (process.argv.length != 4) {
        console.log("node <script> <input_file> <output_dir>");
        process.exit(1);
    }

    const data = loadData(process.argv[2]);
    const entries = processFile(data);
    const rust = generateRustCode(entries);
    const mnemonics = generateMnemonicsEnum(entries);
    const extensions = generateExtensionsEnum(entries);
    const dir = process.argv[3];

    writeFileSync(`${dir}/data/instructions.rs`, rust);
    writeFileSync(`${dir}/model/mnemonic.rs`, mnemonics);
    writeFileSync(`${dir}/model/extension.rs`, extensions);

} catch (e) {
    console.error("ERROR", e);
}
