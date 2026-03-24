# vortexdb/__init__.py

from vortexdb.client import VortexDB
from vortexdb.models import (
    DenseVector,
    Payload,
    Point,
    Similarity,
)
from vortexdb.exceptions import (
    VortexDBError,
    AuthenticationError,
    NotFoundError,
    InvalidArgumentError,
    TimeoutError,
    ServiceUnavailableError,
    InternalServerError,
)

__all__ = [
    "VortexDB",
    "DenseVector",
    "Payload",
    "Point",
    "Similarity",
    "VortexDBError",
    "AuthenticationError",
    "NotFoundError",
    "InvalidArgumentError",
    "TimeoutError",
    "ServiceUnavailableError",
    "InternalServerError",
]
