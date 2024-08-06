run:
	docker start rust-container

stop:
	docker stop rust-container

exec:
	docker exec -it rust-container /bin/bash