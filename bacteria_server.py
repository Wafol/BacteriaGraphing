from fastapi import FastAPI, WebSocket
import asyncio
import random

app = FastAPI()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    
    ################GENERATING BACTERIA DATA#####################
    bac_data = []
    num_of_cells = 10000

    for i in range(0, num_of_cells):
        fr = 0
        to = 1000
        
        one_bac_data = []
        for j in range(7):
            one_bac_data.append(random.randint(fr, to).to_bytes(4, byteorder='big'))

        bac_data.append(one_bac_data)


    indx = 0

    while True:
        #data = await websocket.receive_text()
        
        #await websocket.send_text(f"Message received: {data}")

        if (indx > num_of_cells - 1):
            break

        await websocket.send_bytes(bac_data[indx])
        indx += 1

        await asyncio.sleep(1)