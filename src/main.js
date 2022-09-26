const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  // greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.greet = greet;

let pokerPath = new Map([
  ["hearts_A", "/assets/pokers/hearts_A.jpg"],
  ["hearts_2", "/assets/pokers/hearts_2.jpg"],
  ["hearts_3", "/assets/pokers/hearts_3.jpg"],
  ["hearts_4", "/assets/pokers/hearts_4.jpg"],
  ["hearts_5", "/assets/pokers/hearts_5.jpg"],
  ["hearts_6", "/assets/pokers/hearts_6.jpg"],
  ["hearts_7", "/assets/pokers/hearts_7.jpg"],
  ["hearts_8", "/assets/pokers/hearts_8.jpg"],
  ["hearts_9", "/assets/pokers/hearts_9.jpg"],
  ["hearts_10", "/assets/pokers/hearts_10.jpg"],
  ["hearts_J", "/assets/pokers/hearts_J.jpg"],
  ["hearts_Q", "/assets/pokers/hearts_Q.jpg"],
  ["hearts_K", "/assets/pokers/hearts_K.jpg"],
  ["plum_A", "/assets/pokers/plum_A.jpg"],
  ["plum_2", "/assets/pokers/plum_2.jpg"],
  ["plum_3", "/assets/pokers/plum_3.jpg"],
  ["plum_4", "/assets/pokers/plum_4.jpg"],
  ["plum_5", "/assets/pokers/plum_5.jpg"],
  ["plum_6", "/assets/pokers/plum_6.jpg"],
  ["plum_7", "/assets/pokers/plum_7.jpg"],
  ["plum_8", "/assets/pokers/plum_8.jpg"],
  ["plum_9", "/assets/pokers/plum_9.jpg"],
  ["plum_10", "/assets/pokers/plum_10.jpg"],
  ["plum_J", "/assets/pokers/plum_J.jpg"],
  ["plum_Q", "/assets/pokers/plum_Q.jpg"],
  ["plum_K", "/assets/pokers/plum_K.jpg"],
  ["spades_A", "/assets/pokers/spades_A.jpg"],
  ["spades_2", "/assets/pokers/spades_2.jpg"],
  ["spades_3", "/assets/pokers/spades_3.jpg"],
  ["spades_4", "/assets/pokers/spades_4.jpg"],
  ["spades_5", "/assets/pokers/spades_5.jpg"],
  ["spades_6", "/assets/pokers/spades_6.jpg"],
  ["spades_7", "/assets/pokers/spades_7.jpg"],
  ["spades_8", "/assets/pokers/spades_8.jpg"],
  ["spades_9", "/assets/pokers/spades_9.jpg"],
  ["spades_10", "/assets/pokers/spades_10.jpg"],
  ["spades_J", "/assets/pokers/spades_J.jpg"],
  ["spades_Q", "/assets/pokers/spades_Q.jpg"],
  ["spades_K", "/assets/pokers/spades_K.jpg"],
  ["square_A", "/assets/pokers/square_A.jpg"],
  ["square_2", "/assets/pokers/square_2.jpg"],
  ["square_3", "/assets/pokers/square_3.jpg"],
  ["square_4", "/assets/pokers/square_4.jpg"],
  ["square_5", "/assets/pokers/square_5.jpg"],
  ["square_6", "/assets/pokers/square_6.jpg"],
  ["square_7", "/assets/pokers/square_7.jpg"],
  ["square_8", "/assets/pokers/square_8.jpg"],
  ["square_9", "/assets/pokers/square_9.jpg"],
  ["square_10", "/assets/pokers/square_10.jpg"],
  ["square_J", "/assets/pokers/square_J.jpg"],
  ["square_Q", "/assets/pokers/square_Q.jpg"],
  ["square_K", "/assets/pokers/square_K.jpg"],
  ["King", "/assets/pokers/King.jpg"],
  ["Queen", "/assets/pokers/Queen.jpg"],
]);

function getPath(name: string) :string  {
  return pokerPath[name]
}