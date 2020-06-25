pipeline {
    agent { docker { image 'rust:1.44' } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build --release > log.txt'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test >> log.txt'
            }
        }
    }
    post {
        always {
            echo 'The Enraged Magic Carp has...'
        }
        success {
            echo 'Succeeded!'
        }
        failure {
            echo 'Failed.'
            sh 'cat log.txt'
        }
    }
}