20min
=====

.. image:: https://gitlab.com/grauwoelfchen/20min/badges/master/pipeline.svg
   :target: https://gitlab.com/grauwoelfchen/20min/commits/master

.. image:: https://gitlab.com/grauwoelfchen/20min/badges/master/coverage.svg
   :target: https://gitlab.com/grauwoelfchen/20min/commits/master

|

.. raw:: html

   <del>News von jetzt!</del>

This is simple working timer.


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

   % make cov


License
-------


.. code:: text

   20min
   Copyright 2018 Yasuhiro Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
