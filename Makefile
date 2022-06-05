build:
	docker build -t web-service .

run:
	docker run -p 8080:8080 web-service
