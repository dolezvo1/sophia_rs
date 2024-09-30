# Sophia

A Rust toolkit for RDF and Linked Data.

[![Actions Status](https://github.com/pchampin/sophia_rs/actions/workflows/lint_and_test.yml/badge.svg)](https://github.com/pchampin/sophia_rs/actions)
[![Latest Version](https://img.shields.io/crates/v/sophia.svg)](https://crates.io/crates/sophia)
[![Documentation](https://docs.rs/sophia/badge.svg)](https://docs.rs/sophia/)

It comprises the following crates:

* [`sophia_api`] defines a generic API for RDF and linked data,
  as a set of core traits and types;
  more precisely, it provides traits for describing
  - terms, triples and quads,
  - graphs and datasets,
  - parsers and serializers
* [`sophia_iri`] provides functions, types and traits for validating and resolving IRIs.
* [`sophia_inmem`] defines in-memory implementations of the `Graph` and `Dataset` traits from `sophia_api`.
* [`sophia_term`] defines various implementations of the `Term` trait from `sophia_api`.
* [`sophia_turtle`] provides parsers and serializers for the Turtle-family of concrete syntaxes.
* [`sophia_xml`] provides parsers and serializers for RDF/XML.
* [`sophia_jsonld`] provides preliminary support for JSON-LD.
* [`sophia_c14n`] implements [RDF canonicalization].
* [`sophia_resource`] provides a resource-centric API.
* [`sophia_rio`] is a lower-level crate, used by the ones above. 

and finally:
* [`sophia`] is the “all-inclusive” crate,
  re-exporting symbols from all the crates above.
  (actually, `sophia_xml` is only available if the `xml` feature is enabled)

In addition to the [API documentation](https://docs.rs/sophia/),
a high-level [user documentation](https://pchampin.github.io/sophia_rs/) is available (although not quite complete yet).
  
## Licence

[CECILL-B] (compatible with BSD)

## Citation

When using Sophia, please use the following citation:

> Champin, P.-A. (2020) ‘Sophia: A Linked Data and Semantic Web toolkit for Rust’, in Wilde, E. and Amundsen, M. (eds). The Web Conference 2020: Developers Track, Taipei, TW. Available at: https://www2020devtrack.github.io/site/schedule.

Bibtex:
```bibtex
@misc{champin_sophia_2020,
        title = {{Sophia: A Linked Data and Semantic Web toolkit for Rust},
        author = {Champin, Pierre-Antoine},
        howpublished = {{The Web Conference 2020: Developers Track}},
        address = {Taipei, TW},
        editor = {Wilde, Erik and Amundsen, Mike},
        month = apr,
        year = {2020},
        language = {en},
        url = {https://www2020devtrack.github.io/site/schedule}
}
```

## Third-party crates

The following third-party crates are using or extending Sophia

* [`hdt`](https://crates.io/crates/hdt) provides an implementation of Sophia's traits based on the [HDT](https://www.rdfhdt.org/) format.
* [`manas`](https://crates.io/crates/manas) is a modular framework for implementing [Solid](https://solidproject.org/) compatible servers
* [`nanopub`](https://crates.io/crates/nanopub) is a toolkit for managing [nanopublications](https://nanopub.net/)

## History

An outdated comparison of Sophia with other RDF libraries is still available
[here](https://github.com/pchampin/sophia_benchmark/blob/master/benchmark_results.ipynb).


[`sophia_api`]: https://crates.io/crates/sophia_api
[`sophia_iri`]: https://crates.io/crates/sophia_iri
[`sophia_term`]: https://crates.io/crates/sophia_term
[`sophia_inmem`]: https://crates.io/crates/sophia_inmem
[`sophia_term`]: https://crates.io/crates/sophia_inmem
[`sophia_turtle`]: https://crates.io/crates/sophia_turtle
[`sophia_xml`]: https://crates.io/crates/sophia_xml
[`sophia_jsonld`]: https://crates.io/crates/sophia_jsonld
[`sophia_c14n`]: https://crates.io/crates/sophia_c14n
[`sophia_resource`]: https://crates.io/crates/sophia_resource
[`sophia_rio`]: https://crates.io/crates/sophia_rio
[`sophia`]: https://crates.io/crates/sophia
[CECILL-B]: https://cecill.info/licences/Licence_CeCILL-B_V1-en.html
[RDF test-suite]: https://github.com/w3c/rdf-tests/
[JSON-LD test-suite]: https://github.com/w3c/json-ld-api/
[RDF canonicalization]: https://www.w3.org/TR/rdf-canon/
