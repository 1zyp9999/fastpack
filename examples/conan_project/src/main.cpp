#include <iostream>
#include <fmt/format.h>
#include <nlohmann/json.hpp>

int main() {
    std::cout << "========================================" << std::endl;
    std::cout << "   FastPack Conan Example" << std::endl;
    std::cout << "========================================" << std::endl;
    std::cout << std::endl;

    std::cout << fmt::format("{}", "This is a C++ application built with Conan!") << std::endl;
    std::cout << std::endl;

    nlohmann::json features = {
        {"features", {
            "Conan package manager",
            "fmt formatting library",
            "nlohmann/json library",
            "Cross-platform support",
            "Ultra-fast packaging"
        }}
    };

    std::cout << "Features:" << std::endl;
    for (const auto& feature : features["features"]) {
        std::cout << "  • " << feature.get<std::string>() << std::endl;
    }

    std::cout << std::endl;
    std::cout << "FastPack makes packaging your Conan projects" << std::endl;
    std::cout << "10x faster than traditional tools!" << std::endl;
    std::cout << std::endl;
    std::cout << "========================================" << std::endl;

    return 0;
}