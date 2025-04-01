# **_CLIP-Vectorizer_**

This is openAI's CLIP model based API that creates text and image vector-embeddings to be stored and query a vector database.

## **_Steps To run on localhost using Docker_**

- Make sure Docker is installed and running (and using WSL2 engine if in windows).

- Follow the steps given in [Nvidia docs](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) to install nvidia drivers for your distribution (WSL for windows).

- If CUDA drivers are not present or GPU access is not provided to the container, then it will automatically default to computing on CPU.

- This command builds the image to be run inside a container

  > `docker build -t vectorizer .`

- Run the program inside a container using
  for linux or CUDA supported devices
  > `docker run -it --gpus all -p 5000:8080 vectorizer`  
  > for mac or non CUDA supported devices
  > `docker run -it -p 5000:8080 vectorizer`

## **_API routes_**

- `/vectors`

  > Post route for sending text to be embedded in JSON format.  
  > Example Input JSON:  
  > {  
  > &emsp;"text" : "Your text here",  
  > }

- `/vectors_img`

  > Post route for sending images to be embedded in form-data format.  
  > Example Input Form-data:  
  > Key : file | Value : (Your image file)
