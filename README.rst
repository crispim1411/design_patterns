Padrões de Projeto
===================
Estudo de Design Patterns em Rust

Observer
---------
Permite incluir observadores de um certo tópico. Ao notificar é iterado uma lista de observadores contida dentro do tópico e disparado em cada um seu método de notificação

.. figure:: UML/observer-uml.png

Strategy
---------
Permite incluir formas de processamento. O contexto possui um tipo abstrato estratégia que recebe as implementações concretas.

.. figure:: UML/strategy-uml.png

Considerações
==============
- Weak: Evitar Reference Cycle
- dyn: Objeto Trait
- Rc: Downgrade para Weak
