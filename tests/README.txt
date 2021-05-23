--- General notes ---

* Don't forget to run build.sh to compile the smart contracts used in testing! (Changes in neutron-star-rt may need running rebuild_hard.sh to link properly!)

* When multiple data types are handled in sequence (e.g. when testing functions with type variants), the preferred order for integer types are first u8 trough u64, followed by i8 trough i64. Keeping this standard helps readability. 

--- Naming guidelines ---

(These mostly apply to simple integration tests that act more like unit tests but for components too integrated into the ecosystem to actually be tested separately, E.g. hypervisor and neutron-star functionality. Similar naming schemes are encouraged for other test types too though!)

* Each distinct "feature" that is part of a Neutron "component" should have its own file in testing root named "[component]_[feature]" (e.g. hypervisor_costack for hypervisor costack operators). 

* Smart contracts used in a testing file normally go in the contracts/default_env subrepo, and should be named [component]_[feature]_[brief description of function]". If a smart contract file is placed in the src/bin/ folder it will automatically compile to a binary with the same name, which is the simplest approach for non-complex tests (All subject to future reorganization if things get cluttered. Additional subrepos can also easily be created if new sets of imports are needed)

* Tests should generally follow the naming scheme "test_[what it tests]. Negtests that are derivatives of a test should be named negtest_[what it tests]_[what it does wrong]". 


