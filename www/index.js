import { draw } from "../pkg/BacteriaGraphing";
import { clck } from "../pkg/BacteriaGraphing";


function sleep1(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/** Main entry point */
export async function main() {
    let start = Date.now();
    let graph_time = 0;
    //setTimeout(draw, 2);
    draw();
    let timeTaken = Date.now() - start;

    console.log("Total time taken : " + timeTaken + " milliseconds");
    console.log("Graphing time:" + graph_time + " milliseconds");
    
    /*
    for (let i = 0; i < 50; i++) {
        await sleep1(1000);
  
    }
    */

}
