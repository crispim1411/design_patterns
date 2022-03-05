Padrões de Projeto
===================
Estudo de Design Patterns em Rust

Observer
---------
Permite incluir observadores de um certo tópico
.. figure:: UML/observer-uml.png

Strategy
---------
Permite incluir formas de processamento  
.. figure:: UML/strategy-uml.png

Considerações
==============
- Weak: Evitar Reference Cycle
- dyn: Objeto Trait
- Rc: Downgrade para Weak
