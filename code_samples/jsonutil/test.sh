curl -X POST -H "Content-Type: application/json" -d '{"json": {"nome": "João","idade": 30,"endereco": {"rua": "Rua A","cidade": "São Paulo"},"hobbies": ["futebol", "leitura", "música"]}}' http://localhost:3000/json/set 


{"nome": "João","idade": 30,"endereco": {"rua": "Rua A","cidade": "São Paulo"},"hobbies": ["futebol", "leitura", "música"]}