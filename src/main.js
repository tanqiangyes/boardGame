const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let bulls = await invoke("greet",{ num: Number(greetInputEl.value) })
      .then((message) => {
        return message;
      })
      .catch((error) => {
        alert(error);
        console.error(error);
        return [];
      });
  let myHTML = '';
  console.log(bulls);
  let num = 1;
  for (let bull of bulls) {
    myHTML += '<div id="bull' + num + '">';
    for (let plate of bull) {
      myHTML += '<img src="' + pokerPath.get(plate.pcolor+plate.pvalue) + '" value="' + plate.pvalue + '">';
    }
    myHTML += '<button onclick="cal(this)" value=\'' + JSON.stringify(bull) +'\'>calniu</button></div>';
    num++;
  }
  greetMsgEl.innerHTML = myHTML;
}

window.greet = greet;

async function cal(my) {
  console.log(my.value)
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  my.innerHTML = await invoke("cal", {bull: JSON.parse(my.value)})
      .then((message) => {
        return message;
      })
      .catch((error) => {
        alert(error);
        console.error(error);
        return false;
      });
}
window.cal = cal;

let pokerPath = new Map([
  ["HeartsCardA", "/assets/pokers/HeartsCardA.jpg"],
  ["HeartsCard2", "/assets/pokers/HeartsCard2.jpg"],
  ["HeartsCard3", "/assets/pokers/HeartsCard3.jpg"],
  ["HeartsCard4", "/assets/pokers/HeartsCard4.jpg"],
  ["HeartsCard5", "/assets/pokers/HeartsCard5.jpg"],
  ["HeartsCard6", "/assets/pokers/HeartsCard6.jpg"],
  ["HeartsCard7", "/assets/pokers/HeartsCard7.jpg"],
  ["HeartsCard8", "/assets/pokers/HeartsCard8.jpg"],
  ["HeartsCard9", "/assets/pokers/HeartsCard9.jpg"],
  ["HeartsCard10", "/assets/pokers/HeartsCard10.jpg"],
  ["HeartsCardJ", "/assets/pokers/HeartsCardJ.jpg"],
  ["HeartsCardQ", "/assets/pokers/HeartsCardQ.jpg"],
  ["HeartsCardK", "/assets/pokers/HeartsCardK.jpg"],
  ["PlumCardA", "/assets/pokers/PlumCardA.jpg"],
  ["PlumCard2", "/assets/pokers/PlumCard2.jpg"],
  ["PlumCard3", "/assets/pokers/PlumCard3.jpg"],
  ["PlumCard4", "/assets/pokers/PlumCard4.jpg"],
  ["PlumCard5", "/assets/pokers/PlumCard5.jpg"],
  ["PlumCard6", "/assets/pokers/PlumCard6.jpg"],
  ["PlumCard7", "/assets/pokers/PlumCard7.jpg"],
  ["PlumCard8", "/assets/pokers/PlumCard8.jpg"],
  ["PlumCard9", "/assets/pokers/PlumCard9.jpg"],
  ["PlumCard10", "/assets/pokers/PlumCard10.jpg"],
  ["PlumCardJ", "/assets/pokers/PlumCardJ.jpg"],
  ["PlumCardQ", "/assets/pokers/PlumCardQ.jpg"],
  ["PlumCardK", "/assets/pokers/PlumCardK.jpg"],
  ["SpadesCardA", "/assets/pokers/SpadesCardA.jpg"],
  ["SpadesCard2", "/assets/pokers/SpadesCard2.jpg"],
  ["SpadesCard3", "/assets/pokers/SpadesCard3.jpg"],
  ["SpadesCard4", "/assets/pokers/SpadesCard4.jpg"],
  ["SpadesCard5", "/assets/pokers/SpadesCard5.jpg"],
  ["SpadesCard6", "/assets/pokers/SpadesCard6.jpg"],
  ["SpadesCard7", "/assets/pokers/SpadesCard7.jpg"],
  ["SpadesCard8", "/assets/pokers/SpadesCard8.jpg"],
  ["SpadesCard9", "/assets/pokers/SpadesCard9.jpg"],
  ["SpadesCard10", "/assets/pokers/SpadesCard10.jpg"],
  ["SpadesCardJ", "/assets/pokers/SpadesCardJ.jpg"],
  ["SpadesCardQ", "/assets/pokers/SpadesCardQ.jpg"],
  ["SpadesCardK", "/assets/pokers/SpadesCardK.jpg"],
  ["SquareCardA", "/assets/pokers/SquareCardA.jpg"],
  ["SquareCard2", "/assets/pokers/SquareCard2.jpg"],
  ["SquareCard3", "/assets/pokers/SquareCard3.jpg"],
  ["SquareCard4", "/assets/pokers/SquareCard4.jpg"],
  ["SquareCard5", "/assets/pokers/SquareCard5.jpg"],
  ["SquareCard6", "/assets/pokers/SquareCard6.jpg"],
  ["SquareCard7", "/assets/pokers/SquareCard7.jpg"],
  ["SquareCard8", "/assets/pokers/SquareCard8.jpg"],
  ["SquareCard9", "/assets/pokers/SquareCard9.jpg"],
  ["SquareCard10", "/assets/pokers/SquareCard10.jpg"],
  ["SquareCardJ", "/assets/pokers/SquareCardJ.jpg"],
  ["SquareCardQ", "/assets/pokers/SquareCardQ.jpg"],
  ["SquareCardK", "/assets/pokers/SquareCardK.jpg"],
  ["KingCardKing", "/assets/pokers/KingCardKing.jpg"],
  ["QueenCardQueen", "/assets/pokers/QueenCardQueen.jpg"],
]);

// function getPath(name: string) :string  {
//   return pokerPath[name]
// }