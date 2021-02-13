"""
This file offers the methods to automatically retrieve the graph soc-digg.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-06 12:29:13.109374

The undirected graph soc-digg has 770799 nodes and 5907132 unweighted edges,
of which none are self-loops. The graph is extremely sparse as it has a
density of 0.00002 and is connected, as it has a single component. The
graph median node degree is 2, the mean node degree is 15.33, and the node
degree mode is 1. The top 5 most central nodes are 117150 (degree 17643),
117441 (degree 10602), 117149 (degree 10597), 116859 (degree 7870) and
117151 (degree 7767).


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

@article{zafarani2014users,
        title={Users Joining Multiple Sites: Distributions and Patterns},
        author={Zafarani, Reza and Liu, Huan},
        booktitle = {Proceedings of the 8th International AAAI Conference on Weblogs and Social Media (ICWSM'14)},
        year={2014}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import SocDigg

    # Then load the graph
    graph = SocDigg()

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


def SocDigg(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository"
) -> EnsmallenGraph:
    """Return new instance of the soc-digg graph.

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
    Instace of soc-digg graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-06 12:29:13.109374
	
	The undirected graph soc-digg has 770799 nodes and 5907132 unweighted edges,
	of which none are self-loops. The graph is extremely sparse as it has a
	density of 0.00002 and is connected, as it has a single component. The
	graph median node degree is 2, the mean node degree is 15.33, and the node
	degree mode is 1. The top 5 most central nodes are 117150 (degree 17643),
	117441 (degree 10602), 117149 (degree 10597), 116859 (degree 7870) and
	117151 (degree 7767).
	

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
	
	@article{zafarani2014users,
	        title={Users Joining Multiple Sites: Distributions and Patterns},
	        author={Zafarani, Reza and Liu, Huan},
	        booktitle = {Proceedings of the 8th International AAAI Conference on Weblogs and Social Media (ICWSM'14)},
	        year={2014}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import SocDigg
	
	    # Then load the graph
	    graph = SocDigg()
	
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
        "SocDigg",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        callbacks=[]
        dataset="networkrepository"
    )()
