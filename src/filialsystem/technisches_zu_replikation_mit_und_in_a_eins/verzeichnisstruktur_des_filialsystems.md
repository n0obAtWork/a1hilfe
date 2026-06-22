# Verzeichnisstruktur des Filialsystems

<!-- source: https://amic.de/hilfe/_Repli_Verzeichnisstruktur.htm -->

Aufbau der im Filialsystem verwendeten Verzeichnisstruktur:

 **..\\Aeins**

 **|**

 **|___ \\dbrexp**

 **|**

 **|___ \\Log**

 **| |**

 **| |___ \\alte_DBRLogs**

 **| |** 

 **|___ \\BST1**

 **|**

 **|___ \\BST2**

So sieht die Verzeichnisstruktur in einem laufenden Replikations-/Filialsystem aus. Die Ordner „**Log**“, „**alte_DBRLogs**“ und die Ordner der Betriebsstätten (hier „**BST1**“ und „**BST2**“), werden von der Prozedur „**AMIC_remote_schedule()**“ auf ihre Existenz hin überprüft und ggf. angelegt.

Der Ordner **„dbrexp“** muss als vorbereitender Schritt manuell im Aeins-Verzeichnis wie o.a. angelegt werden.
