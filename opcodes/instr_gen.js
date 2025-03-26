import { readFileSync, writeFileSync } from "fs";

const valid_masks = new Set([
    "0xfe00707f",
    "0x707f",
    "0x7f",
    // "0xfc00707f"
]);

const loadData = (file) => {
    const data = readFileSync(file, { encoding: 'utf8', flag: 'r' });
    return JSON.parse(data);
}

const processFile = (isa) => {
    return Object.entries(isa)
        .map(processEntry)
        .filter(x => !!x);
};

const processEntry = ([key, val]) => {
    const { mask } = val;
    if (!valid_masks.has(mask)) {
        console.log(`Unprocessed instruction: ${key}`);
        return null;
    }

    const res = {
        mnemonic: key,
        extension: val.extension.map(e => e.replace(/^.+_/, "")),
        arch: val.extension[0].indexOf("rv64") == 0 ? 64 : 32,
        key: parseKey(val.encoding)
    };

    return res;
};

const parseKey = encoding => {
    return (parseInt(encoding.replace(/-/g, ""), 2) >> 2);
};

const processDef = def => {
    const key = def.key.toString(16);
    const ext_raw = def.extension[0];
    const ext = String(ext_raw[0]).toUpperCase() + ext_raw.slice(1);
    const rust = `        (0x${key}, ("${def.mnemonic}", EXT::${ext}, ${def.arch})),`;
    return rust;
}

const getRustCode = entries => {
    const header = `
// this is generated code, don't modify it manually!

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::model::RISCVExtension as EXT;

type Row = (&'static str, EXT, u8);

pub (crate) static INSTRUCTIONS: Lazy<HashMap<u16, Row>> = Lazy::new(|| {
    HashMap::from([
`;

    const footer = `
    ])
});
`;

    const lines = entries
        .map(processDef)
        .reduce((str, line) => `${str}\n${line}`, "");


    return header + lines + footer;
}

try {

    if (process.argv.length != 4) {
        console.log("node <script> <input_file> <output_file>");
        process.exit(1);
    }

    const data = loadData(process.argv[2]);
    const entries = processFile(data);
    const rust = getRustCode(entries);

    writeFileSync(process.argv[3], rust);

} catch(e) {
    console.error("ERROR", e);
}
