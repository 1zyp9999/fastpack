#include <iostream>
#include <string>
#include <vector>

class FastPackDemo {
public:
    void run() {
        std::cout << "========================================" << std::endl;
        std::cout << "   FastPack Example Application" << std::endl;
        std::cout << "========================================" << std::endl;
        std::cout << std::endl;
        
        std::cout << "This is a demo application packaged with FastPack!" << std::endl;
        std::cout << std::endl;
        
        std::vector<std::string> features = {
            "Ultra-fast packaging",
            "Multi-threaded compression",
            "Cross-platform support",
            "Easy-to-use GUI",
            "Automatic build integration"
        };
        
        std::cout << "Features:" << std::endl;
        for (size_t i = 0; i < features.size(); ++i) {
            std::cout << "  " << (i + 1) << ". " << features[i] << std::endl;
        }
        
        std::cout << std::endl;
        std::cout << "FastPack makes packaging your Linux applications" << std::endl;
        std::cout << "10x faster than traditional tools!" << std::endl;
        std::cout << std::endl;
        std::cout << "========================================" << std::endl;
    }
};

int main() {
    FastPackDemo demo;
    demo.run();
    return 0;
}