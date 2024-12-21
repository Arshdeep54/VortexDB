FROM python

WORKDIR /src

COPY  . /src

RUN pip3 install torch --index-url https://download.pytorch.org/whl/cu124

RUN pip3 install -r requirements.txt

RUN apt-get update

EXPOSE 8080

CMD ["uvicorn","app.main:app","--host", "0.0.0.0", "--port", "8080"]