//
//  ContentView.swift
//  test-rust-ios
//
//  Created by Dương Thành Đạt on 30/10/2023.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            let rustGreetings = RustGreetings()
            Text("\(rustGreetings.sayHello(to: "dcm"))")
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
