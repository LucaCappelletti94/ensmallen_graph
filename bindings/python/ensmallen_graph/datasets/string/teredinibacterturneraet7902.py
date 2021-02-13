"""
This file offers the methods to automatically retrieve the graph Teredinibacter turnerae T7902.

The graph is automatically retrieved from the STRING repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-02 22:18:21.502604

The undirected graph Teredinibacter turnerae T7902 has 4068 nodes and 535990
weighted edges, of which none are self-loops. The graph is dense as it
has a density of 0.06479 and has 25 connected components, where the component
with most nodes has 4017 nodes and the component with the least nodes has
2 nodes. The graph median node degree is 227, the mean node degree is 263.52,
and the node degree mode is 1. The top 5 most central nodes are 1056820.KB900700_gene1144
(degree 1853), 1056820.KB900649_gene2898 (degree 1672), 1056820.KB900630_gene1448
(degree 1540), 1056820.KB900641_gene598 (degree 1527) and 1056820.KB900649_gene2897
(degree 1511).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import TeredinibacterTurneraeT7902

    # Then load the graph
    graph = TeredinibacterTurneraeT7902()

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


def TeredinibacterTurneraeT7902(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Teredinibacter turnerae T7902 graph.

    The graph is automatically retrieved from the STRING repository. 

	

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
    Instace of Teredinibacter turnerae T7902 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-02 22:18:21.502604
	
	The undirected graph Teredinibacter turnerae T7902 has 4068 nodes and 535990
	weighted edges, of which none are self-loops. The graph is dense as it
	has a density of 0.06479 and has 25 connected components, where the component
	with most nodes has 4017 nodes and the component with the least nodes has
	2 nodes. The graph median node degree is 227, the mean node degree is 263.52,
	and the node degree mode is 1. The top 5 most central nodes are 1056820.KB900700_gene1144
	(degree 1853), 1056820.KB900649_gene2898 (degree 1672), 1056820.KB900630_gene1448
	(degree 1540), 1056820.KB900641_gene598 (degree 1527) and 1056820.KB900649_gene2897
	(degree 1511).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@article{szklarczyk2019string,
	    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
	    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
	    journal={Nucleic acids research},
	    volume={47},
	    number={D1},
	    pages={D607--D613},
	    year={2019},
	    publisher={Oxford University Press}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.string import TeredinibacterTurneraeT7902
	
	    # Then load the graph
	    graph = TeredinibacterTurneraeT7902()
	
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
        "TeredinibacterTurneraeT7902",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="string"
    )()
