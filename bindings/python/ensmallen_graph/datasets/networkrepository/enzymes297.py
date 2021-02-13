"""
This file offers the methods to automatically retrieve the graph ENZYMES297.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-04 08:39:18.267478

The undirected graph ENZYMES297 has 122 nodes with 2 different node types:
2 (nodes number 102) and 1 (nodes number 20), of which 1 are singletons,
and 149 unweighted edges, of which none are self-loops. The graph is dense
as it has a density of 0.02019 and has 2 connected components, where the
component with most nodes has 121 nodes and the component with the least
nodes has a single node. The graph median node degree is 2, the mean node
degree is 2.44 and the node degree mode is 2. The top 5 most central nodes
are 20 (degree 7), 12 (degree 6), 122 (degree 5), 97 (degree 5) and 18
(degree 5).


References
---------------------
Please cite the following if you use the data:

@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import Enzymes297

    # Then load the graph
    graph = Enzymes297()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Wether to show a loading bar.
        verbose=True
    )

    # Remember that, if you need, you can enable the memory-time trade-offs:
    train_graph.enable(
        vector_sources=True,
        vector_destinations=True,
        vector_outbounds=True
    )

    # Consider using the methods made available in the Embiggen package
    # to run graph embedding or link prediction tasks.
"""

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def Enzymes297(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the ENZYMES297 graph.

    The graph is automatically retrieved from the NetworkRepository repository. 

	

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.

    Returns
    -----------------------
    Instace of ENZYMES297 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-04 08:39:18.267478
	
	The undirected graph ENZYMES297 has 122 nodes with 2 different node types:
	2 (nodes number 102) and 1 (nodes number 20), of which 1 are singletons,
	and 149 unweighted edges, of which none are self-loops. The graph is dense
	as it has a density of 0.02019 and has 2 connected components, where the
	component with most nodes has 121 nodes and the component with the least
	nodes has a single node. The graph median node degree is 2, the mean node
	degree is 2.44 and the node degree mode is 2. The top 5 most central nodes
	are 20 (degree 7), 12 (degree 6), 122 (degree 5), 97 (degree 5) and 18
	(degree 5).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import Enzymes297
	
	    # Then load the graph
	    graph = Enzymes297()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Wether to show a loading bar.
	        verbose=True
	    )
	
	    # Remember that, if you need, you can enable the memory-time trade-offs:
	    train_graph.enable(
	        vector_sources=True,
	        vector_destinations=True,
	        vector_outbounds=True
	    )
	
	    # Consider using the methods made available in the Embiggen package
	    # to run graph embedding or link prediction tasks.
    """
    return AutomaticallyRetrievedGraph(
        "Enzymes297",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
