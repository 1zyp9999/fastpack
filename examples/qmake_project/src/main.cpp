#include <QApplication>
#include <QLabel>
#include <QVBoxLayout>
#include <QWidget>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QWidget window;
    QVBoxLayout *layout = new QVBoxLayout(&window);

    QLabel *title = new QLabel("FastPack QMake Example");
    title->setAlignment(Qt::AlignCenter);
    title->setStyleSheet("font-size: 24px; font-weight: bold; color: #00d9ff;");

    QLabel *description = new QLabel("This is a Qt application packaged with FastPack!");
    description->setAlignment(Qt::AlignCenter);
    description->setStyleSheet("font-size: 14px; color: #e0e0e0;");

    QLabel *features = new QLabel(
        "Features:\n"
        "• QMake build system\n"
        "• Qt framework\n"
        "• Cross-platform support\n"
        "• Ultra-fast packaging"
    );
    features->setAlignment(Qt::AlignLeft);
    features->setStyleSheet("font-size: 12px; color: #b0b0b0;");

    layout->addWidget(title);
    layout->addSpacing(20);
    layout->addWidget(description);
    layout->addSpacing(20);
    layout->addWidget(features);

    window.setWindowTitle("QMake Example");
    window.resize(400, 300);
    window.show();

    return app.exec();
}