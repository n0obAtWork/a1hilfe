# Bekannte bzw. gelöste Problembereiche

<!-- source: https://amic.de/hilfe/bekanntebzwgelsteproblembereic.htm -->

Die bisher umgestellten Datenbanken haben folgende Probleme gezeigt:

- Benutzereinrichtungen im Rollenbereich der Sybase 12 besitzen Fremdserverzuordnung.
  - Hier handelt es sich um fehleingerichtete Zuordnungen zu Sybase Elementen, die nicht korrekt in der Datenbankstruktur hinterlegt sind, diese Fehlstrukturen werden entfernt.
    - View Einrichtungen, die veraltet sind
  - Es kommt immer wieder vor, dass eine Datenbankview auf veraltete Informationen zugreifen möchte, die eine korrekte Ausführung der View nicht zulässt. View, die als „veraltet“ erkannt werden, werden aus dem System entfernt. Es gibt hierzu entsprechende Fehlerprotokolleinträge.
