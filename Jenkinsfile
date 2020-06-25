pipeline {
    agent { docker { image 'rust' } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build --release'
            }
        }
    }  
}