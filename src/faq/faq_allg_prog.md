# Allgemeine Programmfunktionen

<details>
  <summary>Frage: Was passiert, wenn man einen Bediener löscht? Erhält er lediglich ein Kennzeichen?</h3></summary>
  
  <p class="answer">Antwort:</p>

  Mit der Funktion „Bediener löschen“ werden die Einträge des Bedieners irreversibel aus verschiedenen Tabellen in A.eins gelöscht! Im Bedienerstamm bleibt der Bediener bestehen und wird als inaktiv gekennzeichnet. Eine Wiederstellung des Bedieners ist nicht möglich! Auch eine „Speichern unter“-Funktion steht in diesem Fall nicht zur Verfügung.

  Möchte man lediglich erreichen, dass sich der Bediener nicht mehr in A.eins anmelden kann, dann trägt man im Bedienerstamm im Feld „Sperre“ ein „Ja“ ein. Eine Reaktivierung ist über die Aufhebung des Sperrkennzeichens jederzeit möglich.
</details>

<details>
  <summary>Frage: Ein Mitarbeiter sieht keine oder nicht alle archivierten Belege.</summary>
  
  <p class="answer">Antwort:</p>
  
  Hierbei handelt es sich i.d.R. um eine Frage von Berechtigungen. Diese werden je Bedienerklasse vergeben. In den archivierten Dokumenten ist die Bedienerklasse desjenigen Mitarbeiters eingetragen, der das Dokument archiviert hat. Die Anzeige des Dokuments im Archiv ist nur für Bediener freigeschaltet, die einer mit dieser Berechtigung ausgestatteten Bedienerklasse angehören. Die Erteilung dieser Berechtigungen wird unter dem Direktsprung BDKL eingestellt. Dort markiert man die betreffende Bedienerklasse, der Ansichtsrechte erteilt werden sollen, und fügt mit “Ändern” (F5) auf dem Reiter “Formulararchiv” die Bedienerklassen hinzu. 

  Beispiel: Hat Herr Müller ein Dokument archiviert und gehört er der Bedienerklasse 200 an, dann muss in der Bedienerklasse 200 auch unter BDKL bei “erlaubte Bedienerklassen für das Formulararchiv” die 200 eingetragen sein, damit er das Dokument sehen kann. Soll auch Frau Schmidt aus der Bedienerklasse 300 dieses Dokument sehen können, so muss in der Bedienerklasse 300 unter BDKL bei “erlaubte Bedienerklassen für das Formulararchiv” ebenfalls die 200 eingetragen sein.
</details>