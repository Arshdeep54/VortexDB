import os
import json
from functools import wraps

PIPE_PATH = "/tmp/vector_pipe"

# Ensure the named pipe exists
if not os.path.exists(PIPE_PATH):
    os.mkfifo(PIPE_PATH)

def vector_output(func):
    """
    Decorator to validate that the embedding model consistently returns vectors of the same dimension.
    """
    reference_dimension = None  # Now it's within the decorator's scope

    @wraps(func)
    def wrapper(*args, **kwargs):
        nonlocal reference_dimension
        
        vector = func(*args, **kwargs)  # Call the original function
        
        # Validate vector type
        if not isinstance(vector, (list, tuple)) or not all(isinstance(v, (float, int)) for v in vector):
            raise ValueError("Output is not a valid vector")
        
        # Set the reference dimension if it's the first call
        if reference_dimension is None:
            reference_dimension = len(vector)
        else:
            # Check if the current vector's dimension matches the reference dimension
            if len(vector) != reference_dimension:
                raise ValueError(f"Output vector dimension {len(vector)} does not match the reference dimension {reference_dimension}")
        
        # Write vector to named pipe
        with open(PIPE_PATH, 'w') as pipe:
            json.dump({"vector": vector}, pipe)
            pipe.flush()
        
        return vector  # Return the original vector as well (optional)
    
    return wrapper