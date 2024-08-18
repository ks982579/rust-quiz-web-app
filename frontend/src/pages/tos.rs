//! frontend/src/pages/tos.rs
//! For displaying to the user the Terms of Service for registering an account with application.
use crate::components::Footer;
use leptos::*;
use leptos_router::A;

/// To Display the Terms of Service to user
#[component]
pub fn TermsOfService() -> impl IntoView {
    // -- Create Signals --
    // -- Create References --
    // -- Use Context --

    // -- Render View --
    view! {
        <div
            class:fill-screen=true
        >
            <header>
                <h1>"Kev's Quiz App"</h1>
                <p>
                    <b>"Disclaimer"</b>
                    ": This website is a university project for educational purposes only. "
                    "Please do not enter any sensitive, personal, or confidential information into the system. "
                    "Use this site at your own risk, understanding it is a student project developed with limited time and resources."
                </p>
            </header>
            <main>
                <section>
                    <h2>"Kev's Quiz App: Terms of Service"</h2>
                    <A href="/new-user">"Create user account"</A> " | "
                    <A href="/home">"Back to Home"</A>
                    <article>
                        <p>
                            "Welcom to Kev's Quiz App, your digital study companion!"
                            " Before diving into a world of creating quizzes, please take a moment to read through our Terms of Service."
                            " By accessing or using our website, you are agreeing to abiding by these terms, so let's get started."
                        </p>
                        <p>
                            "Last Updated: 11th of August, 2024"
                        </p>
                        <ol>
                            <li>
                                <h3>"User Account"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "To unlock the full potential of this application, you must create an account."
                                            " Think of this as your personal key to a new Kingdom of Knowledge."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "You are responsible for safeguarding your account credentials."
                                            " It is not recommended sharing your credentials nor making them public in any manner."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "You must be at least 13 years of age to create an account."
                                            " If you are younger than this, although we admire your enthusiasm, please wait a until you are the required age before joining our platform."
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"Quiz Crafting: Content Creation"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "When forging (creating) questions and quizzes, you are essentially becomming a mini-professor."
                                            " As this is a great power, it comes with great responsibility."
                                            " Please ensure that your content is:"
                                            <ul>
                                                <li>"Original (do not copy other's content)."</li>
                                                <li>"Accurate (as to the best or your knowledge)"</li>
                                                <li>"Not offensive or harmful to others."</li>
                                            </ul>
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "By posting a quiz, your are granting Kev's Quiz App a non-exclusive license to use, display, and distribute it within our platform."
                                            " But don't worry, we will not claim it as our own creation!"
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"Knowledge Consumption: Using the Platform"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "With an account, feel free to take quizzes, learn, and grow your brain!"
                                            " However, please do not:"
                                            <ul>
                                                <li>"Attempt to break, hack, or outsmart our system in any way."</li>
                                                <li>"Use bots or automated tools."</li>
                                                <li>"Share quiz answers publicly."</li>
                                            </ul>
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "By posting a quiz, your are granting Kev's Quiz App a non-exclusive license to use, display, and distribute it within our platform."
                                            " But don't worry, we will not claim it as our own creation!"
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"Respect Your Fellow Scholars"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "Treat other users with respect."
                                            " Strictly no bullying, harassment, racism, or any negative vibes allowed."
                                            " This is a safe place for positive learning experiences!"
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "If you spot any inappropriate content or behaviour, please report it immediately."
                                            " Think of yourslef as a guardian of knowledge."
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"The Fine Print: Liability and Warranties"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "While we strive to make this application (Kev's Quiz App) as amazing as possible, we cannot guarantee it will be perfect 100% of the time."
                                            " We provide this service \"as is\" without any warranties."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "We are not responsible for any brain explosions due to excessive learning!"
                                            " That is, please use this platform responsibly and take breaks when needed."
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"Amendments: Keeping Up With the Times"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "These Terms of Service are subject to change at the discression of the owner."
                                            " We retain the right to update these terms as needed from time to time."
                                            " We will notify users of the platform of any significant changes, but it is also a good idea to check back occasionally."
                                            " Think of it as a pop quiz on our terms."
                                        </p>
                                    </li>
                                </ol>
                            </li>
                            <li>
                                <h3>"Termination: An End to One's Learning Journey"</h3>
                                <ol
                                    style="list-style-type: upper-roman"
                                >
                                    <li>
                                        <p>
                                            "Any user may stop using our application (Kev's Quiz App) at anytime."
                                            " But  we will, of course, be sad to see you go."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "We also reserve the right to suspend or terminate any accounts that violate these terms."
                                            " Please do not make us use these extreme powers."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "If a user believes their account has been incorrectly suspended or terminated, they can appeal the decision."
                                            " However, any decision made after the appeal is final."
                                        </p>
                                    </li>
                                    <li>
                                        <p>
                                            "There is not really a termination process in place at this time, so a terminated account will be deleted permanently from our database."
                                            " At this time, this action is not recoverable."
                                        </p>
                                    </li>
                                </ol>
                            </li>
                        </ol>
                        <p>
                            "By using this platform, you acknowledge that you have read, understood, and agreed to these terms."
                            " Now go forth on your learning journey and conquer the world of knowledge!"
                        </p>
                    </article>
                </section>
            </main>
            <Footer />
        </div>
    }
}
