FROM python:3.8-alpine

WORKDIR /app

COPY ./Pipfile ./Pipfile.lock ./main.py ./

RUN apk upgrade --no-cache && apk add gcc musl-dev make

RUN pip install --no-cache-dir pipenv==2020.8.13 && pipenv lock --requirements > requirements.txt && pip install -r requirements.txt

USER 1000
EXPOSE 3000

CMD ["uvicorn", "main:app", "--port", "3000"]
