em:
	@docker build -t robota-aggregate .

up:
	@docker-composer up -d

donw:
	@docker-composer down
