# Projet tutoré 2k22 - cuisine

## Histoire
Un jour, je regardais la télé avec ma compagne quand soudain nos ventres ont crié famine, le problème quoi cuisine avec les ingrédients que l'on a ? L'application a pour but de répondre à la question "mais qu'est qu'on va manger ce soir ?".

## Description

Le produit prend la forme d'une API collaborative ou chacun peux y ajouter des recette de cuisine. cette API va communiquer avec une base de données d’indexation (elastic). L'autre partie intéressante est que les utilisateurs pourront renseigné les produit qu'il ont et L’API trouvera la recette la plus pertinente.

## Gestion de projet

Le suivi de l'avencement du projet se fera sur Trello sous la forme d'un [Kanban](https://trello.com/b/SXOiDUvl/projet-tutor%C3%A9-2k22-cuisine).


## Stack technique 

- ElasticSearch à été choisi pour étre la base de donne. En effet ces capacité d'indexation et d'aggrégation vont étre utile pour recherche des recettes cohérante 
- L'API serait développé en Rust sequi me premettre de découvir se langage d'avenir
- Les données de sortie de la base de données sera au format json

## Modèle de données

## Route de L'API 
