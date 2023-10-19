val scala3Version = "3.3.0"

lazy val root = project
  .in(file("."))
  .settings(
    name := "lab1_1.2",
    version := "0.1.0-SNAPSHOT",

    scalaVersion := scala3Version,

    libraryDependencies += "org.scalameta" %% "munit" % "0.7.29" % Test
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.11" % Test
  )
