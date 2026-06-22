# Lösung

<!-- source: https://amic.de/hilfe/lsung.htm -->

Die Pascal-Skripte (aber auch beliebige andere Module in Aeins) können so aufgebaut werden, dass sie die individuell unterschiedlichen Parameter aus der Datenbank lesen. Dadurch kann gewährleistet werden, dass ein und dasselbe Skript durch dynamische Parametrisierung bei vielen verschiedenen Kunden unter sehr unterschiedlichen Bedingungen funktioniert.

Beispiel: Soll eine ASCII-Datei eingelesen werden, so ist durch das Skript nicht festgelegt, welche Daten an welcher Position im Skript stehen. Dies ist „von außen“ durch spezielle Datenbankrelationen steuerbar und somit individuell an jede Aeins-Installation anpassbar.
