20min
=====

.. image:: https://gitlab.com/grauwoelfchen/20min/badges/trunk/pipeline.svg
   :target: https://gitlab.com/grauwoelfchen/20min/commits/trunk

.. image:: https://gitlab.com/grauwoelfchen/20min/badges/trunk/coverage.svg
   :target: https://gitlab.com/grauwoelfchen/20min/commits/trunk

.. image:: https://img.shields.io/crates/v/twenty-minutes?label=crates&style=flat
   :target: https://crates.io/crates/twenty-minutes

.. image:: https://docs.rs/twenty-minutes/badge.svg
   :target: https://docs.rs/crate/twenty-minutes

|

.. raw:: html

   <del>News von jetzt!</del>

``20min`` is a command line working timer.

Install
-------

.. code:: zsh

   % cargo install twenty-minutes


Usage
-----

.. code:: zsh

   # 900 secs work and 300 secs rest (same as default)
   % 20min -w 15 -r 5
       Working [==============================================================] 100%
      Finished work [unoptimized + progressbar] targes(+s) in 900 secs
       Resting [===================================================-----------] 80%


.. code:: zsh

   # see 20min --help for the details
   % 20min
   % 20min 15,5
   % 20min 15 5
   % 20min --work 15 --rest 5
   % 20min -w 15


Build
-----

Check ``make help``

.. code:: zsh

   # debug build
   % make build


Test
-----

See ``rust-lang/cargo#1924``

.. code:: zsh

   % make test
   % make test:coverage


License
-------


.. code:: text

   20min
   Copyright 2018-2020 Yasuhiro Яша Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
