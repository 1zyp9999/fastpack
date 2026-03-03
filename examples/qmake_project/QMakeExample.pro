QT += core gui widgets

TARGET = QMakeExample
TEMPLATE = app

SOURCES += src/main.cpp

# Installation
target.path = /usr/local/bin
INSTALLS += target