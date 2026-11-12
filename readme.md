#  Class Misser
## Projet programmation réseau

### Lancer le jeux:
#### 1) Installer rust
Un lien vers la documentation:  
https://rust-lang.org/fr/tools/install/  

#### 2) Installer le jeux
```bash
git clone https://github.com/Gazerbim/ClassMisser.git
```

#### 3) Compiler le jeux
```bash
cd ClassMisser  
cd client  
cargo build  
```
```bash
cd ../server  
cargo build  
```

#### 4) Lancer le jeux
```bash
cargo run  
```
On change de terminal: 
```bash
cd ../client  
cargo run <username> <password> --host <host> --action <action to do>  
```

Le nom d'utilisateur et le mot de passe sont obligatoires pour lancer un client.  
L'option 'host' peut être omise et vaut 127.0.0.1:9001 par défaut. (penser à préciser le port).  
L'option 'action' peut être omise pour se connecter (signin), mais est utile pour créer un compte (signup).  
Exemples:  
```bash
cargo run j1 j1  
cargo run utilisateur mdp --action signup  
cargo run j1 j1 --host 192.168.1.38:9001  
```

### Jouer:
Pour jouer il faut au minimum deux clients et un serveur.  
Pour pouvoir se connecter, il est possible de créer des comptes. Sinon, les comptes j1, j2, j3 et j4 sont également disponnibles (avec, comme mot de passe, respectivement, j1, j2, j3 et j4).  

Pour lancer une partie, un joueur doit créer un lobby. Les autres joueurs peuvent en suite rejoindre ce même lobby. Le premier joueur détient le lobby: c'est lui qui lance la partie.  

### Touches:
#### Général:
Flèches pour se déplacer.  
Entrée pour sélectionner des options.  
Esc pour quitter.  
#### En jeux
Tab pour changer son focus entre le jeux et le chat.  
Espace pour sélectionner une pièce.
