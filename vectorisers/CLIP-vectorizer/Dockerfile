FROM python:3.11

RUN apt-get update && apt-get install -y build-essential python3-dev libjpeg-dev zlib1g-dev

WORKDIR /src

COPY  . /src

RUN pip3 install -r requirements.txt

EXPOSE 8080

CMD ["uvicorn","app.main:app","--host", "0.0.0.0", "--port", "8080"]