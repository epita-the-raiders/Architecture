# [S4] Project
## Membres du goupe
### Chef de projet
> Maël ROUSTIT <mael.roustit@epita.fr><br/>
GitHub : [Neogduh](https://github.com/Neogduh)

### Collaborateurs
> Axel OURY <axel.oury@epita.fr><br/>
GitHub : [AxelOury](https://github.com/AxelOury)

> Damien MARRASSE <damien.marrasse@epita.fr><br/>
GitHub : [Dindam331](https://github.com/Dindam331)

> Romain D'ANGE BOURGUIGNON <romain.d-ange-bourguignon@epita.fr><br/>
GitHub : [Rom1DB](https://github.com/Rom1DB)

## Cahier des charges de `Raider's Architecture`
![Logo de Raider's Architecture](assets/icon.png)</br></br>
Pour notre projet libre de S4, nous avons eu l'idée de programmer un créateur de plans de maisons et d'appartements dans le langage de programmation Rust. Ce programme sera conçu pour Linux via une installation Cargo.

## Pré-requis et outils
Nous allons utiliser la librairie graphique [Slint](https://crates.io/crates/slint) afin de créer notre interface graphique.

## Details
### Presentation du projet
Notre projet "Raider's Architecture" est un logiciel qui vous permet de générer les plans d'un appartement ou d'une maison après lui avoir donné certaines contraintes, du simple nombre de pièces à la surface minimale et maximale de chaque pièce en passant par la plomberie ou l'électricité. Il est possible ou non de trouver une ou plusieurs solutions pour aménager votre espace.

### Pourquoi ce projet ?
Nous avons eu un certain nombre d'idées de projets, comme un traducteur de langage texte-signe, un logiciel de conception de jeux comme Unity, et d'autres petites idées que nous n'avons pas encore approfondies. Nous nous sommes alors demandé ce qui pourrait présenter un réel intérêt algorithmique, tout en étant utile à tous les membres de notre groupe. C'est alors que nous avons eu l'idée d'un explorateur de fichiers pour Linux, mais le défi n'était pas assez grand. Comme nous aimons et jouons de la musique, mais que nous n'avons pas une oreille absolue, nous nous sommes demandé s'il était possible de développer un programme qui écouterait et écrirait de la musique. Mais cette fois, la promesse était trop grande et nous ne pouvions pas la tenir. Nous nous sommes donc tournés vers notre nouvelle idée, qui avait trait à l'architecture.

### Pourquoi "Raider's Architecture" ?
Notre groupe s'appelle "The Raiders" en référence au jeu Tomb Raider, car nous aimons les jeux vidéo et ce fut une véritable aventure que de trouver un sujet... Puis, comme le sujet était l'architecture, le nom "Raider's Architecture" nous est venu naturellement.

## Déploiement
|Déploiement||
|---:|:---|
|**Langage**|Rust|
|**Platforme visée**|Linux|

> [!TIP]
> _Si nous en avons le temps, nous aimerions produire une version pour Windows_

## Fonctionnalités et qualités du projet
### Requirements
<br/>

#### Que **DOIT** faire le projet ?<br/>
* Générer des plans
> Le projet se base d'abord sur le fait que l'on peut créer un plan. Générer 1 ou plusieurs images qui sont des plans plus ou moins complexes du futur bâtiment.

* Une interface "user-friendly"
> Dans la mesure où le principe est que toute personne qui veut utiliser le logiciel doit pouvoir le faire, que ce soit au niveau de l'architecture ou non. Nous allons donc mettre en évidence les contraintes de base, mais il sera toujours possible de complexifier le problème à tout moment.

* Un algorithme
> Il est évident que nous devons concevoir un ou plusieurs algorithmes qui nous permettront de réaliser le projet. Tout d'abord, un algorithme de génération de plans, qui variera selon les cas, par exemple dans le cas d'un appartement ou d'une maison.

<br/>

#### Que **DEVRAIT** faire le projet ?<br/>
* Autres algorithmes
> L'objectif étant d'optimiser et de proposer un maximum de contraintes, nous souhaiterions ajouter à notre programme un système de génération de plans pour des considérations supplémentaires telles que la simulation des coûts ou la planification du travail. En plus, bien sûr, de la complexification et de l'optimisation de l'algorithme principal.

<br/>

#### Que **POURRAIT** faire le projet ?<br/>
* Algorithmes additionnel
> Toujours dans le but d'optimiser et d'améliorer l'algorithme, nous pourrions essayer de mettre en œuvre l'optimisation énergétique et d'autres détails.

* Gestionnaire de projet
> Si nous en avons le temps, nous voulons créer un menu avec lequel nous pourrons gérer différents projets, y compris un système de sauvegarde/téléchargement des paramètres, par exemple.

* Windows version
> Si nous avons vraiment le temps, nous aimerions déployer le logiciel sur Windows à l'aide d'un fichier .exe.

## Liste des contraintes
> Contraintes obligatoires
- L'utilisateur pourra choisir le nombre de pièces, que ce soit au global de la maison ou par type de pièce.
- L'utilisateur pourra choisir la taille de la maison et de chaque pièce individuellement, en termes de minimum, maximum ou fixe (l'algorithme traitera les erreurs, par exemple, demander un salon de 500 m² et une maison de 400 m² ne sera pas possible).
- L'utilisateur pourra demander des maisons de plain-pied ou à étage, imposant ainsi un placement d'escalier par l'algorithme.
- L'utilisateur pourra choisir l'orientation de la maison (plein sud pour la lumière, par exemple).
- L'utilisateur pourra décider s'il veut des portes coulissantes ou battantes, ce qui impactera le design final.
- L'utilisateur pourra choisir le nombre de fenêtres, que ce soit par pièce ou sur la maison globale (min, max, fixe).
- L'utilisateur pourra décider de placer la maison sur un terrain (encore une fois, l'algorithme traitera les erreurs).

> Contraintes facultatives
- La gestion des murs porteurs devrait être possible, pour créer une maison en réaménageant une existante.
- La maison créée sera optimisée énergétiquement pour consommer moins.
- Les circuits d'eau et électriques seront pensés pour être bien placés, avec tout ce qui a besoin d'eau sur le même mur, par exemple.
- Ajout de pièces diverses : un garage, une piscine, etc...

## Attribution des tâches et estimation de l'achèvement

> [!IMPORTANT]  
> Malheureusement, en raison du temps nécessaire pour trouver un projet acceptable, nous ne sommes pas en mesure de démarrer le projet pour la première soutenance, d'où le 0% suivant.

> [!NOTE]  
> Ce tableau est une esquisse du travail que chaque personne devra diriger. Je m'explique : la(les) personne(s) dans "Manager(s)" sera(seront) celle(s) qui sera(seront) en charge du développement de la tâche, pas nécessairement celle(s) qui devra la réaliser/développer. Nous souhaitons travailler en équipe, donc nous ne travaillerons pas seuls, mais ensemble sous la direction du manager de tâches, qui rendra compte au chef de projet, qui vérifiera avec les derniers que les tâches sont exécutées et le seront dans les délais impartis.

|**Quoi ?**|**Manager(s)**|**Soutenance 1**|**Soutenance 2**|**Soutenance 3**|
|:-:|:-:|:-:|:-:|:-:|
|Algorithmes principaux|Damien/Romain|<span style="color: #FF0000">0%</span>|<span style="color: #99FF00">70%</span>|<span style="color: #00FF00">100%</span>|
|Algorithmes secondaires|Damien/Romain|<span style="color: #FF0000">0%</span>|<span style="color: #FFCC00">40%</span>|<span style="color: #00FF00">100%</span>|
||||||
|Interface|Axel|<span style="color: #FF3300">5%</span>|<span style="color: #FFFF00">50%</span>|<span style="color: #00FF00">100%</span>|
|Gestionnaire de projet|Maël|<span style="color: #FF0000">0%</span>|<span style="color: #FF3300">10%</span>|<span style="color: #00FF00">100%</span>|
||||||
|Site internet|Maël|<span style="color: #FF3300">5%</span>|<span style="color: #33FF00">90%</span>|<span style="color: #00FF00">100%</span>|