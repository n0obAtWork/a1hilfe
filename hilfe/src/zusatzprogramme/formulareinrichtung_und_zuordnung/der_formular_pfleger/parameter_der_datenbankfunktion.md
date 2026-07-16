# Parameter der Datenbankfunktion

<!-- source: https://amic.de/hilfe/parameterderdatenbankfunktion.htm -->

Die Datenbank Funktion muss folgende Struktur aufweisen:

CREATE FUNCTION p_DBFuncNumText (

 in in_ZiffernVorkomma char(15),

 in in_ZiffernNachkomma char(15),

 in in_Vorzeichen integer, // 1 oder -1

 in in_Dezimalstellen integer,

 in in_Betrag numeric(15,6)

 )

 RETURNS char(500)

BEGIN

DECLARE text char(200);

 // Text erzeugen mit der gewünschten Darstellung

 RETURN text;

END

Erläuterung:

 in_ZiffernVorkomma: enthält alle Ziffern vor dem Dezimalpunkt

 in_ZiffernNachkomma: enthält alle Ziffern nach dem Dezimalpunkt

 in_Vorzeichen: ist 1 oder –1 je nach Vorzeichen

 in_Dezimalstellen: wie viel Dezimalstellen sollen dargestellt werden

 in_Betrag: der originale Betrag

 Es wird eine Stringvariable (char) als Rückgabe erwartet.
