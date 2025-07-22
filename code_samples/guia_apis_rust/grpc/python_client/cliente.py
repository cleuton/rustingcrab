import grpc
import tarefa_pb2
import tarefa_pb2_grpc

def executar():
    with grpc.insecure_channel('localhost:50051') as canal:
        stub = tarefa_pb2_grpc.TarefaServiceStub(canal)
        # Lista as tarefas iniciais
        resposta = stub.Listar(tarefa_pb2.ListaTarefasRequest())
        print("Tarefas atuais:", [tarefa.descricao for tarefa in resposta.tarefas])

        # Cria uma nova tarefa
        nova = stub.Criar(tarefa_pb2.CriarTarefaRequest(descricao='Escrever relatório'))
        print("Tarefa criada:", nova.id, nova.descricao, "concluída?", nova.concluida)

        # Lista de novo para ver a inclusão
        resposta_atual = stub.Listar(tarefa_pb2.ListaTarefasRequest())
        print("Tarefas agora:", [t.descricao for t in resposta_atual.tarefas])

if __name__ == '__main__':
    executar()
