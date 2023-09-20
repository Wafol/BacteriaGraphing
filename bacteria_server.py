from fastapi import FastAPI, WebSocket
import asyncio
import random

app = FastAPI()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    
    ################GENERATING RANDOM BACTERIA DATA#####################
    bac_data = []
    num_of_cells = 100000

    for i in range(0, num_of_cells):
        fr = 0
        to = 1000
        
        one_bac_data = []
        for j in range(7):
            one_bac_data.append(random.randint(fr, to).to_bytes(4, byteorder='big')) #the big edian order is important

        bac_data.append(one_bac_data)
    ####################################################################
    
    indx = 0

    while True:
        #await websocket.send_text(f"Message received: {data}")

        if (indx > num_of_cells - 1):
            print("All cells in the generated array were sent. Ending connection. \n")
            break

        await websocket.send_bytes(bac_data[indx])
        indx += 1

        #data = await websocket.receive_text() #use this code when you are waiting for the client response
        await asyncio.sleep(0.0001)

