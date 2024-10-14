# A Multiplayer Hangman Game 
## Todo List

## 1. Set Up Basic Game Structure
- [x] Simple use input printing
- [x] Create blank word display function
- [x] Create main game loop
    - [x] Choosing word
    - [x] Guessing word
    - [x] Fix the pretty printing & the git drama

## 2. Implement Core Game Logic
- [x] Add user input handling for letter guesses
- [x] Implement letter checking against chosen word
- [x] Update display for correct guesses
- [ ] Implement word selection mechanism in Single Player
  - [ ] Hard-coded word list
  - [ ] Random word selection (optional)
- [ ] Track and limit incorrect guesses

## 3. Create ASCII Art
- [ ] Design ASCII art for each hangman state
- [ ] Adding states machine
    - [ ] Implement function to display current hangman state
- [ ] Create victory and game over ASCII displays

## 4. Enhance User Interface
- [ ] Add color with ANSI escape codes 
- [ ] Implement simple animations (is it possible????)
- [ ] Add sound effects if supported (is it fucking possiblleleelierje???!)

## 5. Implement Multiplayer Functionality
- [ ] Create duel mechanism for multiple players
- [ ] Set up multi-threaded server
- [ ] Implement client-server communication
  - [ ] Handle incoming connections
  - [ ] Manage multiple game sessions
- [ ] Synchronize game state across clients
- [ ] Create lobby system for multiple players

## 6. Networking Setup
- [ ] Choose and implement network protocol (e.g., TCP)
- [ ] Set up server to listen on specific IP and port
- [ ] Implement port forwarding (if necessary)
- [ ] Configure firewall for incoming connections

## 7. Client-Side Development
- [ ] Create client program for remote players
- [ ] Implement connection to server
- [ ] Handle game state updates from server
- [ ] Send user inputs to server

## 8. Polish and Finalize
- [ ] Refactor code for better organization
- [ ] Add error handling and input validation
- [ ] Implement scoring system (optional)
- [ ] Create user documentation
- [ ] Test thoroughly, including edge cases

## 9. Deployment
- [ ] Set up server on desired hardware
- [ ] Create startup script or service
- [ ] Implement logging for server events
- [ ] Set up monitoring for server health (optional)

## 10. Post-Launch
- [ ] Gather user feedback
- [ ] Plan and implement updates or expansions
- [ ] Monitor server performance and scale if needed
