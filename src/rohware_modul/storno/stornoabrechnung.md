# Stornoabrechnung:

<!-- source: https://amic.de/hilfe/stornoabrechnung.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung

Direktsprung **[RWBV]**

Den falschen Beleg in der Rohwarenanlieferung markieren. Danach unten links Optionen

Die Erstellung von Stornobelegen erfolgt für Rohwarelieferungen mit der Funktion **Lie. Stornobeleg** und für Rohwareabrechnungen (Abschlag, Folgeabschlag, Finale) mit der Funktion **Abr. Stornobeleg** in den Auswahllisten mit Einzelbelegen des Rohwarebearbeitungsmoduls.

Stornobelege zu Sammeldruckbelegen (Sammelabrechnungen) werden mit der Funktion [**Sammel-Storno erstellen**](../sammelabrechnung/bearbeiten_sammeldruck.md#Sammelstornoerzeugen) in der Auswahllistenvariante **Bearbeiten Sammeldruck** erzeugt.

Für die Funktionen zur Stornobelegerzeugung zu einzelnen Rohwarebelegen gibt es eine Reihe von Rohwareparametern [RWPA] und Einstellungen auf der Steuerungsmaske, die das Verhalten steuern und Einfluss auf bestimmte Werte der erzeugten Stornobelege haben.  
Zunächst bestimmt der Parameter [*Stornobeleg nur nach Fibuübertrag*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_114) ob die Stornobeleg-Erzeugung auch zu nicht gebuchten Belegen erfolgen darf. Handelt es sich bei den zu stornierenden Belegen um Vorgänge aus anderen als dem aktuellen Geschäftsjahr, so bestimmt der Parameter [*Storno alter Belege nach Inventureinspielung*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_151) *,* wie bei Vorliegen einer zwischenzeitlich bereits eingespielten Inventur zu verfahren ist.  
Der Parameter [*Belegdatum bei Stornobelegen*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_131) legt fest, ob die Stornobelege mit dem Belegdatum der Quellbelege erstellt werden sollen oder dieses auf der Steuerungsmaske von Fall zu Fall einzustellen ist.  
Dabei ist zu beachten, dass in der Variante Stornobelegerstellung ohne Kopie des Originalbelegs das ggf. angegebene Belegdatum ebenso wie die ggf. angegebene Periode im selben Geschäftsjahr wie das des zu stornierenden Belegs liegen muss..  
Mit dem Parameter [*Perioden bei Stornobelegen*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_121) wird die Periodenbehandlung der Stornobelege festgelegt. Es können die Originalperioden erhalten bleiben oder die Einstellung auf der Maske vorgenommen werden. Es ist jedoch zu beachten, dass bei unpassenden Angaben von Periode und Jahr automatische Perioden-/Jahr-Anpassungen mit entsprechendem Hinweis vom System vorgenommen werden. Zusätzlich kann für Stornobeleg von Abrechnungen mit dem Parameter [*Storno-Fibuperiode=0*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_175) festgelegt werden, dass die Buchungsperiode für die Finanzbuchhaltung zunächst nicht festgelegt wird, um dieses dann bei der Durchführung des Fibu-Übertrags automatisch neu bestimmen zu können.  
Die Druck-Formularnummer des Stornobelegs berechnet sich aus der Druckformularnummer des Original-Beleges zuzüglich einem Offset, der im Rohwareparameter [RWPA] [*Stornoformularnummer: Offset zum Originalformular*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_84) festgelegt ist. Unter der so bestimmten Nummer sollte dann auch ein Druckformular eingerichtet sein.

 Das Feld **Stornodatumverhalten** ist nur bei dem Wert **Ja** im Feld ***Storno nach Kopie*** änderbar. Standardmäßig steht dort der Wert ***kein manuelles Valutadatum***.

Wenn dieser Wert auf **Stornobeleg mit manuellem Valutadatum** oder **Stornobeleg und Kopie mit manuellem Valutadatum** geändert wird, dann wird das **manuelle Valutadatum** Feld mit dem heutigen Datum vorbelegt und wird bearbeitbar. Falls vorhanden, wird eine in das Feld **manuelle Zahlungsbedingung** eingerichtete Zahlungsbedingung gezogen mit dem Typ **Datum manuell eingebbar** und das Feld wird bearbeitbar.  
Dadurch werden je nach Wert im Feld **Stornodatumverhalten** nur der Stornobeleg oder auch die Belegkopie mit dem manuellem Valutadatum erstellt.

Zusätzlich zur Erstellung von Stornobelegen kann der ursprüngliche Beleg unter dem angegeben Datum kopiert werden. Diese Option ist aber nur freigeschaltet, wenn der Steuerungsparameter [SPA] [*Rohware-Storno mit Quellbeleg-Kopie*](../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/rohwarestorno_mit_quellbeleg_kopie_spa_654.md) mit dem Wert ‚*erlaubt‘* eingestellt ist. Vorbelegt mit dem Wert des Rohwareparameters [*Stornobelege: Vorbelegung: >mit Kopie&lt;*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_171) kann dieses dann auf der Steuerungsmaske festgelegt werden. Wird ein neues Belegdatum angegeben, so gibt es im Maskenfeld ’*Belegdatum und Periode*‘ die Möglichkeit, festzulegen, ob das neue Belegdatum nur für die Kopie oder auch für den Stornobeleg zu gelten hat.  
Eine weitere Option, freigeschaltet durch den Steuerungsparameter [SPA] [*Rohwarestorno mit Alternativen bez. Fibustatus*](../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/rohwarestorno_mit_alternat_bez_fibstatus_spa_655.md) mit dem Wert ‚*erlaubt‘,* ermöglicht unterschiedliche automatische Aktionen hinsichtlich der erfolgten oder nicht erfolgten Übertragung und Buchung in die Finanzbuchhaltung. Das durch den Steuerparameter freigeschaltete Maskenfeld ‚*Stornobeleg erzeugen*‘ erlaubt die Alternativen

‚*immer*‘  
Stornobelegerzeugung erfolgt unabhängig vom Fibu-Status

‚*Nur wenn FIBU Übertrag*‘  
Es wird nur dann ein Stornobeleg erzeugt, wenn der Originalbeleg an die Finanzbuchhaltung übergeben wurde.

‚*Nur wenn FIBU gebucht*‘  
Es wird nur dann ein Stornobeleg erzeugt, wenn der Originalbeleg an die Finanzbuchhaltung übergeben und gebucht wurde. Andernfalls wird ggf. der Primanota-Eintrag gelöscht und die Belege als nicht an die Fibu übertragen gekennzeichnet.

Bezüglich des Einflusses von Bewegungen der Stornobelege und Belegkopien auf Massebilanzen ist die Einstellung des bei der Option **Storno mit Kopie** herangezogenen Rohwareparameters [*Massebilanz bei Storno mit Kopie*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_191) zu berücksichtigen.

Zusätzlich kann eine nach der Stornobelegerzeugung auszuführende Folgefunktion ( Druck, Fibuübertrag, Druck und Fibuübertrag) für die neu erzeugten Belege gewählt werden.
