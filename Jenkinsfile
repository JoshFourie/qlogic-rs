pipeline {
    agent { docker { image 'rust' } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('test-vector') {
            steps {
                sh 'cd vector'
                sh 'cargo test'
            }
        }
    }  
}