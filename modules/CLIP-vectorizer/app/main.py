import time
# import PIL
from PIL import Image
from io import BytesIO
# import requests
from fastapi import FastAPI , File ,Form , UploadFile 
import torch
from torch.nn.functional import normalize
from transformers import CLIPProcessor, CLIPModel
from pydantic import BaseModel

model_id="openai/clip-vit-base-patch32"
torch_dtype = torch.float16

model = CLIPModel.from_pretrained(model_id,torch_dtype=torch_dtype,)
processor = CLIPProcessor.from_pretrained(model_id,clean_up_tokenization_spaces=True)


# if you have cuda set it to the active device
device = "cuda" if torch.cuda.is_available() else "cpu"
print(device)
# to display CUDA device name
# print(torch.cuda.get_device_name(torch.cuda.current_device()))  

# move the model to the device
model.to(device)

app = FastAPI()

class Text_input(BaseModel):
    text: str

@app.post("/vectors")
async def generate_text_embedding(text_input: Text_input ):
    phrase = text_input.text
    # print("Input: ",phrase)
    label_tokens = processor(
        text=phrase,
        padding=True,
        images=None,
        return_tensors='pt'
    ).to(device)
    # encode tokens to sentence embeddings
    label_embeddings = model.get_text_features(**label_tokens)
    
    # normalize the vector embeddings
    label_embeddings = normalize(label_embeddings, p=2, dim=1)
    
    # detach from pytorch gradient computation 
    label_embeddings = label_embeddings.detach().cpu().tolist()
    return {"result":label_embeddings[0]}



@app.post("/vectors_img")
async def generate_image_embedding(file: UploadFile = File(...)):
    # demo_url = "https://www.androidauthority.com/wp-content/uploads/2022/11/twitter-1-scaled-1000w-563h.jpg.webp"
    # img  = Image.open(requests.get(img_url, stream=True).raw)
    
    # Read the file contents as bytes
    file_bytes = await file.read()
    # Wrap the bytes in a BytesIO object
    image_stream = BytesIO(file_bytes)
    # Open the image using PIL
    img = Image.open(image_stream)
    image = processor(
        text=None,
        images=img,
        return_tensors='pt'
    ).to(device)['pixel_values']
    # encode tokens to image embeddings
    image_embeddings = model.get_image_features(image)

    # normalize the vector embeddings    
    image_embeddings = normalize(image_embeddings, p=2, dim=1)
    
    # detach from pytorch gradient computation 
    image_embeddings = image_embeddings.detach().cpu().tolist()
    return {"result":image_embeddings[0]}



def benchmark_text():
    sentences = ["List of test sentences"] # Add your test sentences
    # print(len(sentences))
    lap_times=[]
    rounds=10
    for j in range(rounds):
        start_time = time.time()
        for i in sentences:
            generate_text_embedding(i)
        total_time = time.time()-start_time
        lap_times.append(total_time)
    print("The program while running on",device,"took:"+str(sum(lap_times)/rounds))



def benchmark_image():
    images = ["URLs of images"] #Add your test URLS
    lap_times=[]
    rounds=10
    for j in range(rounds):
        start_time = time.time()
        k=0
        for i in images:
            try:
                generate_image_embedding(i)  # uncomment the img url line before running image benchmark and change the input parameter to a string img_url instead of file also comment all lines related to direct image input
            except Exception as e:
                return print("Can't convert: ",i,"at k = ",k,"\n",e)
            k+=1
        total_time = time.time()-start_time
        lap_times.append(total_time)
    print("The program while running on",device,"took:"+str(sum(lap_times)/rounds))



# benchmark_text()
# benchmark_image() 

    