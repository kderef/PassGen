import { clipboard } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/tauri";

let prev_len: number = 0;

async function generatePassword() {
    let passOutput: HTMLInputElement = document.querySelector("#pass-output")!;
    let passLen: HTMLInputElement = document.querySelector("#pass-length")!;
    let useNums: HTMLInputElement = document.querySelector("#use-numbers")!;
    let useSChars: HTMLInputElement = document.querySelector("#use-schars")!;

    passOutput.value = await invoke("generate_password",
        { len: +passLen.value, useSpecialCharacters: useSChars.checked, useNums: useNums.checked, }
    )
}

window.addEventListener("DOMContentLoaded", () => {
    let el: HTMLInputElement = document.querySelector("#pass-length")!;
    el.oninput = () => {
        let inp: HTMLInputElement = document.querySelector("#pass-length")!;
        let out: HTMLInputElement = document.querySelector('#length-output')!;

        if (+inp.value === prev_len) return;
        prev_len = +inp.value;
        out.textContent = ('Length: ' + inp.value).padEnd(9, ' ');
        generatePassword();
    }

    let chN: HTMLInputElement = document.querySelector("#use-numbers")!;
    let chS: HTMLInputElement = document.querySelector("#use-schars")!;

    chN.onchange = generatePassword;
    chS.onchange = generatePassword;

    let copyButton: HTMLInputElement = document.querySelector("#copy-button")!;

    copyButton.onclick = () => {
        let pass: HTMLInputElement = document.querySelector("#pass-output")!;
        clipboard.writeText(pass.value)
    };

    let regenButton: HTMLInputElement = document.querySelector("#regen-button")!;
    regenButton.onclick = generatePassword;

    // run once on startup
    generatePassword();
});