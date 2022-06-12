FROM python:3.10.5-slim

RUN apt-get update

RUN apt-get install git wget curl iputils-ping net-tools zsh -y

RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

WORKDIR /app

RUN export FLASK_APP=script

RUN pip install flask

ENTRYPOINT flask run --host=0.0.0.0