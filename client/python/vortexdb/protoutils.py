from vortexdb.grpc import vector_db_pb2
from vortexdb.models import DenseVector, Payload, Similarity

def build_insert_request(
    *,
    vector: DenseVector,
    payload: Payload,
) -> vector_db_pb2.InsertVectorRequest:
    return vector_db_pb2.InsertVectorRequest(
        vector=vector.to_proto(),
        payload=payload.to_proto(),
    )

def build_point_id_request(point_id: str) -> vector_db_pb2.PointID:
    return vector_db_pb2.PointID(
        id=vector_db_pb2.UUID(value=point_id)
    )

def build_search_request(
    *,
    vector: DenseVector,
    similarity: Similarity,
    limit: int,
) -> vector_db_pb2.SearchRequest:
    return vector_db_pb2.SearchRequest(
        query_vector=vector.to_proto(),
        similarity=similarity.to_proto(),
        limit=limit,
    )
