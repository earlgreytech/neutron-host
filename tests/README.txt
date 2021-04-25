General notes: 

* Don't forget to run build.sh to compile the smart contracts used in testing! 

* Since parts of contract repos aren't tracked by git parts can be left when switching branches, and this can cause contract build errors. These are harmless, but can be fixed by simply deleting the folder of the offending contract (That doesn't actually exist in your current branch). 

Naming guidelines: 

(These mostly apply to simple integration tests that act more like unit tests, but for components too integrated into the ecosystem to actually be tested separately. E.g. hypervisor and neutron-star functionality. Similar naming and folder schemes are encouraged for other test types too though!)

* Each distinct "feature" that is part of a Neutron "component" should have its own folder in testing root named "[component]_[feature]" (e.g. hypervisor_costack for hypervisor costack operators). 

* A testing folder folder can contain one or many testing contracts in folders named "contract_[name], with each being used in a file/module simply named [name]". (name should briefly describe what the contract does)

* Tests should generally follow the naming scheme "test_[what it tests]. Negtests that are derivatives of a test should be named negtest_[what it tests]_[what it does wrong]". 


