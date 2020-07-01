pipeline {
    agent { 
        kubernetes { 
            label 'qlogic-rs/dev'
            defaultContainer 'rust'
            yaml """
apiVersion: v1
kind: Pod
metadata:
  labels:
    app: jenkins-ci
spec:
  serviceAccountName: default
  containers:
    - name: rust
      image: rust
"""
        } 
    }
    stages {
        stage('build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test --all'
            }
        }
    }  
}
