import { clipboard } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/tauri";

let prev_len: number = 0;

async function generatePassword() {
    let passOutput: HTMLInputElement | null = document.querySelector("#pass-output");
    let passLen: HTMLInputElement | null = document.querySelector("#pass-length");
    let passLenOutput: HTMLInputElement | null = document.querySelector("#length-output")
    let useNums: HTMLInputElement | null = document.querySelector("#use-numbers");
    let useSChars: HTMLInputElement | null = document.querySelector("#use-schars");

    if (passOutput && passLen && passLenOutput && useNums && useSChars) {
        passOutput.value = await invoke("generate_password",
            { len: +passLen.value, useSpecialCharacters: useSChars.checked, useNums: useNums.checked, }
        )
    }
}

window.addEventListener("DOMContentLoaded", () => {
    let el: HTMLInputElement | null = document.querySelector("#pass-length");
    if (el) el.oninput = () => {
        let inp: HTMLInputElement | null = document.querySelector("#pass-length");
        let out: HTMLInputElement | null = document.querySelector('#length-output');

        if (inp && out) {
            if (+inp.value === prev_len) return;
            prev_len = +inp.value;
            if (out) out.textContent = ('Length: ' + inp.value).padEnd(9, ' ');
            generatePassword();
        }
    }

    let chN: HTMLInputElement | null = document.querySelector("#use-numbers");
    let chS: HTMLInputElement | null = document.querySelector("#use-schars");

    if (chN && chS) {
        chN.onchange = generatePassword;
        chS.onchange = generatePassword;
    }

    let copyButton: HTMLInputElement | null = document.querySelector("#copy-button");

    if (copyButton) copyButton.onclick = () => {
        let pass: HTMLInputElement | null = document.querySelector("#pass-output");
        if (pass) clipboard.writeText(pass.value)
    };

    // run once on startup
    generatePassword();
});