// const { invoke } = window.__TAURI__.tauri;

// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document.querySelector("#greet-form").addEventListener("submit", (e) => {
//     e.preventDefault();
//     greet();
//   });
// });


// const { invoke } = window.__TAURI__.tauri;

function bmiCalculator(height, weight) {
  var w = parseInt(weight);
  var h = parseInt(height) / 100;
  return (w / (h * h)).toFixed(2);
}

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

window.addEventListener("DOMContentLoaded", () => {
  const btn = document.querySelector("button");

  btn.addEventListener("click", () => {
    const height = document.querySelector("#bodyHeight").value;
    const weight = document.querySelector("#bodyWeight").value;
    const resultArea = document.querySelector("#resultText");

    if (height != "" && weight != "") {
      resultArea.innerHTML = bmiCalculator(height, weight);
    }
  });
});