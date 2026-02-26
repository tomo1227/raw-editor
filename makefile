.PHONY: up
up:
	docker compose up -d

.PHONY: down
down:
	docker compose down

.PHONY: prune
prune:
	docker system prune -a

.PHONY: delete
delete:
	docker compose down --rmi all --volumes --remove-orphans

.PHONY: log
log:
	docker compose logs -f
