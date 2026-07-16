# Verzeichnisstruktur des Filialsystems

<!-- source: https://amic.de/hilfe/_Repli_Verzeichnisstruktur.htm -->

Aufbau der im Filialsystem verwendeten Verzeichnisstruktur:

 **..\\Aeins**

 **&#124;**

 **&#124;___ \\dbrexp**

 **&#124;**

 **&#124;___ \\Log**

 **&#124; &#124;**

 **&#124; &#124;___ \\alte_DBRLogs**

 **&#124; &#124;** 

 **&#124;___ \\BST1**

 **&#124;**

 **&#124;___ \\BST2**

So sieht die Verzeichnisstruktur in einem laufenden Replikations-/Filialsystem aus. Die Ordner „**Log**“, „**alte_DBRLogs**“ und die Ordner der Betriebsstätten (hier „**BST1**“ und „**BST2**“), werden von der Prozedur „**AMIC_remote_schedule()**“ auf ihre Existenz hin überprüft und ggf. angelegt.

Der Ordner <strong>„dbrexp“</strong> muss als vorbereitender Schritt manuell im Aeins-Verzeichnis wie o.a. angelegt werden.
